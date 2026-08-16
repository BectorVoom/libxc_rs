//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1240/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1240(t10786: f64, t10789: f64, t10873: f64, t10887: f64, t1107: f64, t1108: f64, t1955: f64, t1977: f64, t21146: f64, t26323: f64, t2848: f64, t2849: f64, t30255: f64, t30259: f64, t30261: f64, t3592: f64, t3604: f64, t5835: f64, t5838: f64, t5845: f64, t5903: f64, t721: f64, t7315: f64, t7494: f64, t9203: f64, t9437: f64, t9440: f64, t9443: f64, t9451: f64) -> f64 {
    let t30498 = -t30255 - t30259 + 0.10526802520742363173e2_f64 * t7315 * t9437 - 0.70178683471615754484e1_f64 * t7494 * t9440 - 0.31168546390226634765e3_f64 * t21146 * t9443 - 0.14035736694323150897e2_f64 * t5838 * t10873 * t721 + 0.10526802520742363173e2_f64 * t1977 * t3592 * t2848 + 0.6233709278045326953e3_f64 * t5845 * t10887 * t721 - 0.35089341735807877242e1_f64 * t5903 * t10786 - 0.35089341735807877242e1_f64 * t1955 * t2849 * t3604 - 0.35089341735807877242e1_f64 * t1955 * t1108 * t9203 + 0.51947577317044391277e2_f64 * t5835 * t10789 + 0.51947577317044391277e2_f64 * t1977 * t26323 * t1107 + 0.51947577317044391277e2_f64 * t1977 * t9451 * t2848 - t30261;
    t30498
}
