//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1215/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1215(t1088: f64, t5870: f64, t1100: f64, t5490: f64, t663: f64, t7502: f64, t685: f64, t1096: f64, t1108: f64, t17385: f64, t17605: f64, t17707: f64, t1933: f64, t1941: f64, t1950: f64, t21173: f64, t21179: f64, t21186: f64, t21196: f64, t2796: f64, t2816: f64, t2849: f64, t5820: f64, t5846: f64, t5866: f64, t5874: f64, t5877: f64, t5900: f64, t702: f64, t7324: f64, t7447: f64, t7475: f64) -> (f64, f64) {
    let t21203 = t1088 * t5870;
    let t21212 = t1100 * t5490;
    let t21215 = t7502 * t663;
    let t21217 = 3.0_f64 * t21215 * t685;
    let t21218 = 0.6207121550312808036e4_f64 * t21173 * t17707 * t702 + 3.0_f64 * t7447 * t1933 + 0.96491876992155210402e2_f64 * t21179 * t1941 + 0.17544670867903938621e1_f64 * t1950 * t7475 - t21186 + t21196 + 0.5848223622634646207e0_f64 * t17385 * t1108 + 0.17544670867903938621e1_f64 * t5877 * t2849 + 1.0_f64 * t2796 * t5866 + 0.2069040516770936012e4_f64 * t21203 * t5874 + 1.0_f64 * t17605 * t1096 + 3.0_f64 * t5820 * t2816 + 6.0_f64 * t7324 * t5900 + 0.10254018858216406658e4_f64 * t21212 * t5846 - t21217;
    (t21217, t21218)
}
