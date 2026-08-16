//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3544/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3544(t11922: f64, t11927: f64, t19830: f64, t16055: f64, t19738: f64, t16095: f64, t20100: f64, t43131: f64, t11866: f64, t15691: f64, t15906: f64, t15908: f64, t16025: f64, t16081: f64, t16082: f64, t16098: f64, t16226: f64, t19611: f64, t19745: f64, t19831: f64, t20096: f64, t3117: f64, t3155: f64, t43285: f64, t4907: f64, t54089: f64, t54916: f64, t55182: f64, t65144: f64, t66667: f64, t66766: f64) -> f64 {
    let t67353 = t11927 * t11922 * t19830;
    let t67355 = t19738 * t16055;
    let t67358 = t16095 * t43131 * t20100;
    let t67382 = -0.11433071498151929859e-2_f64 * t55182 + 0.11433071498151929859e-2_f64 * t16226 * t15691 * t3155 * t66667 + 0.57165357490759649296e-3_f64 * t67353 + 0.11433071498151929859e-2_f64 * t67355 - 0.6351706387862183255e-3_f64 * t67358 + 0.11433071498151929859e-2_f64 * t54089 * t20096 + 0.11433071498151929859e-2_f64 * t66766 * t16098 + 0.12862205435420921092e-2_f64 * t16081 * t3117 * t65144 * t16082 - 0.12862205435420921092e-2_f64 * t15906 * t3117 * t65144 * t15908 + 0.42874018118069736972e-3_f64 * t11927 * t3117 * t19611 * t16025 + 0.45732285992607719436e-2_f64 * t54916 * t4907 + 0.85748036236139473944e-3_f64 * t43285 * t19831 - 0.42874018118069736972e-3_f64 * t11866 * t19745;
    t67382
}
