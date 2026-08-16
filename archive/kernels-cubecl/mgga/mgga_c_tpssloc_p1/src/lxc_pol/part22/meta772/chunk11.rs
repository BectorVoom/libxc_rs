//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2643/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2643<F: Float>(t1256: F, t19267: F, t193: F, t27843: F, t336: F, t4700: F, t5091: F, t66897: F, t72104: F, t72106: F, t72138: F, t72201: F, t72203: F, t72207: F, t72209: F, t72211: F, t72213: F, t73852: F, t73885: F, t73919: F) -> F {
    let t73931 = -t72104 - t72106 + t193 * t336 * (t72138 + t73852 + t73885 + t73919) * t1256 - t72201 - t72203 + F::cast_from(6.0_f64) * t4700 * t66897 * t27843 + t72207 - t72209 + t72211 - t72213 - F::cast_from(3.0_f64) * t4700 * t19267 * t5091;
    t73931
}
