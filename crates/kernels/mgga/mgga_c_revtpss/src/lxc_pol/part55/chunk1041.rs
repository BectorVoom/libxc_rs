//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1041/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1041<F: Float>(t31747: F, t31750: F, t31763: F, t2061: F, t7048: F, t8650: F, t31812: F, t8651: F, t886: F, t1955: F, t7398: F, t31828: F) -> (F, F, F, F, F, F, F, F) {
    let t32437 = F::new(0.37645955677973955999e-4) * t31747;
    let t32438 = F::new(0.66934509195437693771e-4) * t31750;
    let t32439 = F::new(0.263521689745817692e-2) * t31763;
    let t32440 = t2061 * t7048;
    let t32441 = t8650 * t32440;
    let t32445 = t31812 * t8651 * t886;
    let t32450 = t1955 * t7398;
    let t32456 = F::new(0.3718732920905101082e-4) * t31828;
    (t32437, t32438, t32439, t32440, t32441, t32445, t32450, t32456)
}
