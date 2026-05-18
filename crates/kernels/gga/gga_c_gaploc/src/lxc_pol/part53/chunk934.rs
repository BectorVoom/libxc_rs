//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 934/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk934<F: Float>(t10012: F, t2684: F, t2925: F, t9438: F, t3005: F, t9800: F, t9829: F, t13142: F, t7416: F, t10054: F, t3040: F, t3267: F, t8556: F) -> (F, F, F, F, F) {
    let t44001 = t2684 * t9438 * t10012 * t2925;
    let t44002 = F::new(0.15976219147466979032e-1) * t44001;
    let t44004 = t9800 * t3005 * t9829;
    let t44005 = F::new(0.19171462976960374838e1) * t44004;
    let t44009 = t7416 * t13142;
    let t44010 = F::new(0.15976219147466979032e-1) * t44009;
    let t44027 = F::new(0.35750489951850426669e0) * t10054 * t3040;
    let t44029 = F::new(0.23833659967900284446e0) * t3267 * t8556;
    (t44002, t44005, t44010, t44027, t44029)
}
