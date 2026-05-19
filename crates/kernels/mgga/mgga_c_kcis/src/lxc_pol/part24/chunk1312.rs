//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1312/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1312<F: Float>(t99904: F, t99906: F, t99908: F, t99910: F, t99912: F, t99914: F, t99917: F, t99919: F, t99921: F, t99923: F, t99925: F, t99927: F, t99929: F, t99931: F, t99933: F, t99935: F, t99937: F, t99939: F, t99941: F) -> F {
    let t101661 = F::cast_from(0.59953703703703703705e-2_f64) * t99904 + F::new(0.4046875e-1) * t99906 - F::new(0.1875e0) * t99908 - F::new(0.625e-1) * t99910 + F::new(0.125e0) * t99912 + F::new(0.375e0) * t99914 + F::new(0.625e-1) * t99917 - F::cast_from(0.89930555555555555557e-2_f64) * t99919 + F::cast_from(0.53958333333333333334e-1_f64) * t99921 + F::cast_from(0.26979166666666666667e-1_f64) * t99923 + F::new(0.20234375e-1) * t99925 - F::cast_from(0.26979166666666666667e-1_f64) * t99927 + F::new(0.1875e0) * t99929 + F::cast_from(0.14388888888888888889e0_f64) * t99931 - F::cast_from(0.20833333333333333333e-1_f64) * t99933 + F::cast_from(0.33333333333333333333e0_f64) * t99935 + F::new(0.4046875e-1) * t99937 - F::new(0.4046875e-1) * t99939 - F::new(0.625e-1) * t99941;
    t101661
}
