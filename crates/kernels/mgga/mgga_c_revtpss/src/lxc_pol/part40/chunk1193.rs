//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1193/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1193<F: Float>(t15809: F, t4872: F, t1042: F, t1011: F, t1063: F, t11753: F, t11756: F, t11763: F, t11866: F, t15782: F, t15787: F, t15791: F, t15796: F, t15804: F, t3127: F, t3241: F, t4892: F, t4907: F, t4916: F, t4920: F) -> (F,) {
    let t15810 = t4872 * t15809;
    let t15811 = t1042 * t15810;
    let t15814 = 0.85748036236139473944e-3 * t4892 * t15782 + 0.42874018118069736972e-3 * t4892 * t15787 - 0.57165357490759649296e-3 * t1063 * t15791 - t15796 - 0.42874018118069736972e-3 * t11866 * t4907 + t11753 / 864.0 + t11756 / 648.0 - t11763 / 432.0 + t3241 * t4916 / 27.0 + t1011 * t15804 / 48.0 - 2.0 / 81.0 * t3241 * t4920 - 0.14291339372689912324e-3 * t3127 * t15811;
    (t15814,)
}
