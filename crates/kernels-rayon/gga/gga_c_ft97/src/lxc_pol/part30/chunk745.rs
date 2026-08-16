//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 745/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk745(t2404: f64, t684: f64, t33436: f64, t17987: f64, t2035: f64, t27521: f64, t27616: f64, t27658: f64, t30671: f64, t33385: f64, t33390: f64, t33399: f64, t33405: f64, t33408: f64, t33413: f64, t33415: f64, t33418: f64, t33424: f64, t33426: f64, t33428: f64, t33434: f64, t33437: f64, t33443: f64, t33445: f64) -> (f64, f64, f64) {
    let t33446 = t2404 * t684;
    let t33447 = t33436 * t33446;
    let t33450 = -0.19762785756235085044e-4_f64 * t17987 * t2035 * t33399 - 0.39601100101559655353e-5_f64 * t27616 * t33405 - 0.68116566383613497688e-3_f64 * t27521 * t33408 + t33413 + 0.11352761063935582948e-3_f64 * t27658 * t33415 + 0.25845121844514357744e-4_f64 * t33418 * t33385 + 0.22227677429409423704e-2_f64 * t30671 * t33390 - 0.68246728907663312894e-4_f64 * t33424 * t33426 * t33428 - 0.17608347349624143343e-1_f64 * t33434 * t33436 * t33437 + t33443 + 0.39129660776942540761e-2_f64 * t33445 * t33447;
    (t33446, t33447, t33450)
}
