//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1253/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1253<F: Float>(t33222: F, t24657: F, t7372: F, t2684: F, t32803: F, t7585: F, t7427: F, t7573: F, t5679: F, t7682: F, t8792: F, t2628: F, t8521: F) -> (F, F, F, F, F, F) {
    let t33223 = F::new(0.19171462976960374838e0) * t33222;
    let t33224 = t24657 * t7372;
    let t33225 = F::new(0.29792074959875355558e-1) * t33224;
    let t33228 = F::new(0.14953741122029092374e3) * t2684 * t7585 * t32803;
    let t33231 = F::new(0.37959496694381542179e3) * t7427 * t7573 * t32803;
    let t33238 = F::new(0.21450293971110256002e1) * t5679 * t8792 * t7682;
    let t33239 = t8521 * t2628;
    (t33223, t33225, t33228, t33231, t33238, t33239)
}
