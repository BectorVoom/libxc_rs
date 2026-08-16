//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3010/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3010(t16219: f64, t3241: f64, t11637: f64, t11672: f64, t11703: f64, t15153: f64, t15850: f64, t15965: f64, t16027: f64, t16095: f64, t3184: f64, t43129: f64, t43133: f64, t43146: f64, t43169: f64, t43285: f64, t43512: f64, t43611: f64, t4891: f64, t4896: f64, t4902: f64) -> f64 {
    let t55033 = t3241 * t16219;
    let t55034 = t55033 / 162.0_f64;
    let t55039 = 0.42874018118069736973e-2_f64 * t16095 * t11703 * t15153 * t11637 + 0.12862205435420921092e-2_f64 * t43512 * t4891 * t4896 - 0.64311027177104605458e-3_f64 * t43611 * t4891 * t4902 + 0.12862205435420921092e-2_f64 * t43285 * t16027 + 0.7145669686344956162e-3_f64 * t15850 * t3184 + 0.28582678745379824648e-3_f64 * t43129 + 0.47637797908966374413e-3_f64 * t43133 + t55034 - 0.14481890564325777822e-1_f64 * t43146 + 0.45732285992607719436e-2_f64 * t11672 * t15965 + 0.57165357490759649295e-3_f64 * t43169;
    t55039
}
