//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3001/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3001<F: Float>(t19639: F, t78900: F, t1043: F, t11774: F, t15689: F, t15700: F, t15701: F, t16052: F, t16226: F, t19864: F, t19982: F, t19986: F, t19992: F, t19997: F, t19998: F, t23931: F, t3117: F, t42781: F, t42872: F, t43069: F, t4787: F, t54388: F, t54414: F, t54509: F, t55141: F, t66114: F, t66306: F, t66542: F, t66644: F, t66647: F, t66655: F, t66660: F, t66686: F, t66777: F, t67052: F, t67458: F, t78812: F) -> (F, F) {
    let t79610 = t78900 * t19639;
    let t79627 = F::cast_from(0.51448821741683684368e-2_f64) * t54509 * t3117 * t78812 * t42872 * t1043 - F::cast_from(0.85748036236139473944e-3_f64) * t67458 * t19986 + F::cast_from(0.42344709252414555034e-4_f64) * t42781 + F::cast_from(0.17149607247227894789e-2_f64) * t66114 * t19998 - F::cast_from(0.11433071498151929859e-2_f64) * t66644 + F::cast_from(0.11433071498151929859e-2_f64) * t66647 + F::cast_from(0.14291339372689912324e-2_f64) * t66542 * t19982 + F::cast_from(0.57165357490759649295e-3_f64) * t66655 + F::cast_from(0.28582678745379824648e-3_f64) * t66660 + t54388 + F::cast_from(0.85748036236139473944e-3_f64) * t43069 * t66306 * t4787 - F::cast_from(0.85748036236139473944e-3_f64) * t55141 * t19864 + F::cast_from(0.85748036236139473944e-3_f64) * t15689 * t15701 * t79610 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t67052 * t4787 + F::cast_from(0.95275595817932748827e-4_f64) * t54414 - F::cast_from(0.85748036236139473944e-3_f64) * t15700 * t66777 * t19992 - F::cast_from(0.68598428988911579157e-2_f64) * t16052 * t23931 + F::cast_from(0.85748036236139473944e-3_f64) * t16226 * t66777 * t19997 + t66686 / F::cast_from(288.0_f64);
    (t79610, t79627)
}
