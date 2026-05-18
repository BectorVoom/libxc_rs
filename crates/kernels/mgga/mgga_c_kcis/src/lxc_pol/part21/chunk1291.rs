//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1291/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1291<F: Float>(t7703: F, t95684: F, t26685: F, t26692: F, t27950: F, t92730: F, t93403: F, t93406: F, t93409: F, t93437: F, t95756: F, t95759: F, t95764: F, t95769: F, t95775: F) -> F {
    let t95779 = F::new(0.46336805555555555556e-3) * t7703 * t95684;
    let t95780 = F::new(0.44218518518518518517e-2) * t95756 - F::new(0.66327777777777777776e-2) * t95759 + F::new(0.16475308641975308642e-2) * t26692 * t27950 - F::new(0.20594135802469135802e-3) * t95764 - F::new(0.15445601851851851852e-3) * t93403 - F::new(0.7722800925925925926e-4) * t93406 - F::new(0.10297067901234567901e-3) * t93409 - F::new(0.556528203125e-3) * t26685 * t95769 - F::new(0.46336805555555555556e-3) * t93437 - F::new(0.92673611111111111113e-3) * t95775 - F::new(0.73697530864197530861e-3) * t92730 - t95779;
    t95780
}
