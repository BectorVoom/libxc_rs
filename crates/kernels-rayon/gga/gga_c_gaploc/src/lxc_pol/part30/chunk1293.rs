//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1293/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1293(t11029: f64, t2087: f64, t4614: f64, t10951: f64, t5782: f64, t2101: f64, t3431: f64, t1890: f64, t3487: f64, t7805: f64, t7810: f64, t107: f64, t10809: f64, t787: f64) -> (f64, f64, f64, f64, f64) {
    let t33282 = 0.18404604457881959845e2_f64 * t2087 * t4614 * t11029;
    let t33284 = 0.18404604457881959845e2_f64 * t5782 * t10951;
    let t33285 = t2101 * t3431;
    let t33289 = t1890 * t3487;
    let t33291 = t7810 * t33289 * t7805;
    let t33292 = 0.19171462976960374838e1_f64 * t33291;
    let t33294 = t787 * t10809 * t107;
    (t33282, t33284, t33285, t33292, t33294)
}
