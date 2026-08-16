//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2989/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2989<F: Float>(t11994: F, t15707: F, t15830: F, t16226: F, t1671: F, t19693: F, t19697: F, t19750: F, t19878: F, t20083: F, t23630: F, t23635: F, t23844: F, t23886: F, t23929: F, t23980: F, t3106: F, t3117: F, t3155: F, t3188: F, t42621: F, t42622: F, t4574: F, t4869: F, t4892: F, t6327: F, t65613: F, t65823: F, t65840: F, t66565: F, t66621: F, t78496: F, t79219: F, t79233: F, t79247: F, t79253: F) -> F {
    let t79255 = -F::cast_from(0.7145669686344956162e-3_f64) * t15707 * t19693 + F::cast_from(0.85748036236139473944e-3_f64) * t11994 * t23635 + F::cast_from(0.64311027177104605458e-3_f64) * t65613 * t1671 + F::cast_from(0.64311027177104605458e-3_f64) * t19697 * t4869 - F::cast_from(0.38110238327173099531e-2_f64) * t3106 * t23844 + F::cast_from(0.57165357490759649296e-3_f64) * t79219 + F::cast_from(0.12862205435420921092e-2_f64) * t19878 * t20083 + F::cast_from(0.76220476654346199061e-2_f64) * t3106 * t23886 - F::cast_from(0.45732285992607719437e-2_f64) * t3106 * t23630 - F::cast_from(0.38110238327173099531e-2_f64) * t15830 * t6327 + F::cast_from(0.63517063878621832552e-3_f64) * t3188 * t23980 + F::cast_from(0.57165357490759649295e-3_f64) * t65823 - F::cast_from(0.42874018118069736972e-3_f64) * t79233 + F::cast_from(0.38586616306262763276e-2_f64) * t66621 * t19750 - F::cast_from(0.12862205435420921092e-2_f64) * t42621 * t3117 * t78496 * t42622 + F::cast_from(0.12862205435420921092e-2_f64) * t4892 * t3117 * t66565 * t23929 + F::cast_from(0.17149607247227894789e-2_f64) * t65840 + F::cast_from(0.14291339372689912324e-2_f64) * t16226 * t79247 * t3155 * t4574 - F::cast_from(0.42874018118069736972e-3_f64) * t79253;
    t79255
}
