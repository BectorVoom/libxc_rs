//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1148/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1148<F: Float>(t10260: F, t10309: F, t42196: F, t44423: F, t44424: F, t44425: F, t44428: F, t44431: F, t47990: F, t47994: F, t47996: F, t48000: F, t48009: F, t48011: F, t48014: F, t48017: F, t48022: F, t4965: F) -> F {
    let t49803 = -F::cast_from(0.85129199786595678799e-5_f64) * t47990 + F::cast_from(0.85129199786595678799e-5_f64) * t47994 + F::cast_from(0.1702583995731913576e-4_f64) * t47996 - F::cast_from(0.5107751987195740728e-4_f64) * t48000 - F::cast_from(0.11974241701863808564e0_f64) * t4965 * t10309 + F::cast_from(0.2727466165424534173e0_f64) * t48009 + F::cast_from(0.5454932330849068346e-1_f64) * t48011 - F::cast_from(0.53337116123857557163e0_f64) * t42196 + F::cast_from(0.8980681276397856423e-1_f64) * t48014 + F::cast_from(0.79828278012425390428e-1_f64) * t4965 * t10260 - t44423 - F::cast_from(0.11974241701863808564e0_f64) * t48017 + t44424 - t44425 + t44428 - t44431 + F::cast_from(0.15965655602485078085e0_f64) * t48022;
    t49803
}
