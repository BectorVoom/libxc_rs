//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1163/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1163<F: Float>(t114: F, t2327: F, t7356: F, t94973: F, t94976: F, t94979: F, t94981: F, t94983: F, t94986: F, t94988: F, t10259: F, t10416: F, t1312: F, t13435: F, t13440: F, t2055: F, t2322: F, t2371: F, t26153: F, t26399: F, t28658: F, t46126: F, t49693: F, t49851: F, t5523: F, t60551: F, t670: F, t7359: F, t7373: F, t95347: F, t95357: F) -> (F, F, F) {
    let t115 = F::new(1.0) < t114;
    let t95371 = t7356 * t2327;
    let t95397 = F::new(308.0) / F::new(27.0) * t94973;
    let t95405 = piecewise3::<F>(t115, F::new(0.0), -t95397 - F::new(22.0) / F::new(3.0) * t94976 - F::new(4.0) * t94979 + F::new(2.0) * t94981 - F::new(3.0) / F::new(2.0) * t94983 + F::new(3.0) / F::new(2.0) * t94986 - t94988 / F::new(4.0));
    let t95408 = F::new(2.0) * t10259 * t7359 + F::new(6.0) * t10416 * t7373 + F::new(2.0) * t1312 * t95405 + F::new(12.0) * t13435 * t7373 + F::new(6.0) * t13440 * t7373 + F::new(2.0) * t2055 * t46126 + F::new(6.0) * t2055 * t49693 + F::new(6.0) * t2055 * t49851 + F::new(2.0) * t2055 * t60551 + F::new(6.0) * t2322 * t26153 + F::new(6.0) * t2371 * t26399 + F::new(6.0) * t2371 * t28658 + F::new(6.0) * t26153 * t5523 + F::new(6.0) * t670 * t95357 + t95347 + F::new(6.0) * t95371;
    (t95371, t95405, t95408)
}
