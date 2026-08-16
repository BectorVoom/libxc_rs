//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1222/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1222<F: Float>(t99904: F, t99906: F, t99908: F, t99910: F, t99912: F, t99914: F, t99917: F, t99919: F, t99921: F, t99923: F, t99925: F, t99927: F, t99929: F, t99931: F, t99933: F, t99935: F, t99937: F, t99939: F, t99941: F) -> F {
    let t99943 = t99904 / F::cast_from(432.0_f64) + t99906 / F::cast_from(64.0_f64) - t99908 / F::cast_from(8.0_f64) - t99910 / F::cast_from(24.0_f64) + t99912 / F::cast_from(12.0_f64) + t99914 / F::cast_from(4.0_f64) + t99917 / F::cast_from(24.0_f64) - t99919 / F::cast_from(288.0_f64) + t99921 / F::cast_from(48.0_f64) + t99923 / F::cast_from(96.0_f64) + t99925 / F::cast_from(128.0_f64) - t99927 / F::cast_from(96.0_f64) + t99929 / F::cast_from(8.0_f64) + t99931 / F::cast_from(18.0_f64) - t99933 / F::cast_from(72.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t99935 + t99937 / F::cast_from(64.0_f64) - t99939 / F::cast_from(64.0_f64) - t99941 / F::cast_from(24.0_f64);
    t99943
}
