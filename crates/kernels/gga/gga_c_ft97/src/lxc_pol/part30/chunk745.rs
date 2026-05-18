//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 745/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk745<F: Float>(t2404: F, t684: F, t33436: F, t17987: F, t2035: F, t27521: F, t27616: F, t27658: F, t30671: F, t33385: F, t33390: F, t33399: F, t33405: F, t33408: F, t33413: F, t33415: F, t33418: F, t33424: F, t33426: F, t33428: F, t33434: F, t33437: F, t33443: F, t33445: F) -> (F, F, F) {
    let t33446 = t2404 * t684;
    let t33447 = t33436 * t33446;
    let t33450 = -F::new(0.19762785756235085044e-4) * t17987 * t2035 * t33399 - F::new(0.39601100101559655353e-5) * t27616 * t33405 - F::new(0.68116566383613497688e-3) * t27521 * t33408 + t33413 + F::new(0.11352761063935582948e-3) * t27658 * t33415 + F::new(0.25845121844514357744e-4) * t33418 * t33385 + F::new(0.22227677429409423704e-2) * t30671 * t33390 - F::new(0.68246728907663312894e-4) * t33424 * t33426 * t33428 - F::new(0.17608347349624143343e-1) * t33434 * t33436 * t33437 + t33443 + F::new(0.39129660776942540761e-2) * t33445 * t33447;
    (t33446, t33447, t33450)
}
