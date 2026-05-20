//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta732 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2502;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2503;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta732<F: Float>(t10985: F, t15017: F, t15045: F, t2435: F, t15048: F, t2471: F, t15008: F, t2439: F, t4469: F, t780: F, t785: F, t213: F, t252: F, t2440: F, t4534: F, t1580: F, t41117: F, t10509: F, t10995: F, t14990: F, t10868: F, t820: F, t844: F, t14701: F, t40731: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50214, t50219, t50221, t50223, t50236, t50240) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2502::<F>(t10985, t15017, t15045, t2435, t15048, t2471, t15008, t2439, t4469, t780, t785, t213, t252);
        let (t50245, t50248, t50253, t50295, t50298) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2503::<F>(t2439, t2440, t4534, t1580, t41117, t10509, t10995, t14990, t10868, t820, t844, t14701, t40731);
    (t50214, t50219, t50221, t50223, t50236, t50240, t50245, t50248, t50253, t50295, t50298)
}
