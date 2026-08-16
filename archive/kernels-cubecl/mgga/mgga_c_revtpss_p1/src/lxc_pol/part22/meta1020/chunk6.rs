//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3544/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3544<F: Float>(t11922: F, t11927: F, t19830: F, t16055: F, t19738: F, t16095: F, t20100: F, t43131: F, t11866: F, t15691: F, t15906: F, t15908: F, t16025: F, t16081: F, t16082: F, t16098: F, t16226: F, t19611: F, t19745: F, t19831: F, t20096: F, t3117: F, t3155: F, t43285: F, t4907: F, t54089: F, t54916: F, t55182: F, t65144: F, t66667: F, t66766: F) -> F {
    let t67353 = t11927 * t11922 * t19830;
    let t67355 = t19738 * t16055;
    let t67358 = t16095 * t43131 * t20100;
    let t67382 = -F::cast_from(0.11433071498151929859e-2_f64) * t55182 + F::cast_from(0.11433071498151929859e-2_f64) * t16226 * t15691 * t3155 * t66667 + F::cast_from(0.57165357490759649296e-3_f64) * t67353 + F::cast_from(0.11433071498151929859e-2_f64) * t67355 - F::cast_from(0.6351706387862183255e-3_f64) * t67358 + F::cast_from(0.11433071498151929859e-2_f64) * t54089 * t20096 + F::cast_from(0.11433071498151929859e-2_f64) * t66766 * t16098 + F::cast_from(0.12862205435420921092e-2_f64) * t16081 * t3117 * t65144 * t16082 - F::cast_from(0.12862205435420921092e-2_f64) * t15906 * t3117 * t65144 * t15908 + F::cast_from(0.42874018118069736972e-3_f64) * t11927 * t3117 * t19611 * t16025 + F::cast_from(0.45732285992607719436e-2_f64) * t54916 * t4907 + F::cast_from(0.85748036236139473944e-3_f64) * t43285 * t19831 - F::cast_from(0.42874018118069736972e-3_f64) * t11866 * t19745;
    t67382
}
