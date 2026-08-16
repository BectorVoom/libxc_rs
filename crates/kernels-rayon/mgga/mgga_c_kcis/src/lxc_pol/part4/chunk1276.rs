//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1276/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1276(t11479: f64, t11482: f64, t16195: f64, t16198: f64, t16201: f64, t16204: f64, t16207: f64, t16210: f64, t16213: f64, t16215: f64, t16219: f64, t11475: f64, t16046: f64, t16052: f64, t16057: f64, t16067: f64, t16071: f64, t16075: f64, t16080: f64, t16084: f64, t16127: f64, t16129: f64, t16132: f64, t16135: f64, t16137: f64, t16142: f64, t16146: f64, t16163: f64, t16165: f64, t16168: f64, t16292: f64, t16306: f64) -> f64 {
    let t16328 = 0.258925e1_f64 * t16195 + 0.16557e0_f64 * t16198 - 0.49671e0_f64 * t16201 - 0.27595e-1_f64 * t16204 - 0.36793333333333333333e-1_f64 * t16207 + 0.11038e0_f64 * t16210 + 0.16557e0_f64 * t16213 + 0.16504875e0_f64 * t16215 - t11479 - t11482 + 0.16557e0_f64 * t16219;
    let t16330 = -0.71747e0_f64 * t16127 - 0.91983333333333333334e-1_f64 * t16129 - 0.412621875e-1_f64 * t16132 - 0.258925e1_f64 * t16135 - 0.1294625e1_f64 * t16137 - 0.22141166666666666666e1_f64 * t16052 - 0.13418888888888888889e0_f64 * t16046 - 0.66228e0_f64 * t16142 - t16292 + 0.36793333333333333334e-1_f64 * t16146 + t16306 + 0.16504875e0_f64 * t16163 + 0.82524375e-1_f64 * t16165 + 0.19419375e1_f64 * t16168 - 0.33547222222222222222e0_f64 * t16057 + 0.80513333333333333333e0_f64 * t16067 - 0.20128333333333333333e0_f64 * t16071 - 0.181155e1_f64 * t16075 - 0.24154e1_f64 * t16080 + 0.60385e0_f64 * t16084 - 0.11038e0_f64 * t11475 + t16328;
    t16330
}
