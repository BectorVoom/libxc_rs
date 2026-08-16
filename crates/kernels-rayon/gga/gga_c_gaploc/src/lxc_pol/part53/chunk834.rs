//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 834/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk834(t41726: f64, t6717: f64, t6914: f64, t12878: f64, t4953: f64, t40073: f64, t40076: f64, t40090: f64, t41596: f64, t447: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41729 = 0.37959496694381542179e3_f64 * t6914 * t6717 * t41726;
    let t41734 = 0.62115540045351614476e2_f64 * t4953 * t12878;
    let t41735 = 0.59584149919750711116e-1_f64 * t40073;
    let t41736 = 0.25561950635947166451e0_f64 * t40076;
    let t41737 = 0.19171462976960374838e1_f64 * t40090;
    let t41738 = t41596 * t447;
    (t41729, t41734, t41735, t41736, t41737, t41738)
}
