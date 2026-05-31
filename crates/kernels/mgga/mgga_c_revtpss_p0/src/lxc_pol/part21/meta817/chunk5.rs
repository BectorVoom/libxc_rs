//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3010/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3010<F: Float>(t16219: F, t3241: F, t11637: F, t11672: F, t11703: F, t15153: F, t15850: F, t15965: F, t16027: F, t16095: F, t3184: F, t43129: F, t43133: F, t43146: F, t43169: F, t43285: F, t43512: F, t43611: F, t4891: F, t4896: F, t4902: F) -> F {
    let t55033 = t3241 * t16219;
    let t55034 = t55033 / F::cast_from(162.0_f64);
    let t55039 = F::cast_from(0.42874018118069736973e-2_f64) * t16095 * t11703 * t15153 * t11637 + F::cast_from(0.12862205435420921092e-2_f64) * t43512 * t4891 * t4896 - F::cast_from(0.64311027177104605458e-3_f64) * t43611 * t4891 * t4902 + F::cast_from(0.12862205435420921092e-2_f64) * t43285 * t16027 + F::cast_from(0.7145669686344956162e-3_f64) * t15850 * t3184 + F::cast_from(0.28582678745379824648e-3_f64) * t43129 + F::cast_from(0.47637797908966374413e-3_f64) * t43133 + t55034 - F::cast_from(0.14481890564325777822e-1_f64) * t43146 + F::cast_from(0.45732285992607719436e-2_f64) * t11672 * t15965 + F::cast_from(0.57165357490759649295e-3_f64) * t43169;
    t55039
}
