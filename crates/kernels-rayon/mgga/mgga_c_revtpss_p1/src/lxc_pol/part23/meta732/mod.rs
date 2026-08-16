//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta732 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2502;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2503;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta732(t10985: f64, t15017: f64, t15045: f64, t2435: f64, t15048: f64, t2471: f64, t15008: f64, t2439: f64, t4469: f64, t780: f64, t785: f64, t213: f64, t252: f64, t2440: f64, t4534: f64, t1580: f64, t41117: f64, t10509: f64, t10995: f64, t14990: f64, t10868: f64, t820: f64, t844: f64, t14701: f64, t40731: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50214, t50219, t50221, t50223, t50236, t50240) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2502(t10985, t15017, t15045, t2435, t15048, t2471, t15008, t2439, t4469, t780, t785, t213, t252);
        let (t50245, t50248, t50253, t50295, t50298) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2503(t2439, t2440, t4534, t1580, t41117, t10509, t10995, t14990, t10868, t820, t844, t14701, t40731);
    (t50214, t50219, t50221, t50223, t50236, t50240, t50245, t50248, t50253, t50295, t50298)
}
