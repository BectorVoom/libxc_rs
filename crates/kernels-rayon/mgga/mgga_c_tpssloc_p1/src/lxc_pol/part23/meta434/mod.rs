//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1273;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta434(t15503: f64, t18356: f64, t18975: f64, t5024: f64, t1174: f64, t21749: f64, t3431: f64, t135: f64, t22011: f64, t18375: f64, t5019: f64, t18329: f64, t4889: f64, t18324: f64, t22136: f64, t15740: f64, t18371: f64, t1222: f64, t22175: f64, t1734: f64, t6218: f64, t22169: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t72632, t72634, t72648, t72669, t72673, t72703) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1273(t15503, t18356, t18975, t5024, t1174, t21749, t3431, t135, t22011, t18375, t5019, t18329, t4889);
        let (t72705, t72708, t72727, t72733, t72767, t72798) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1274(t18324, t4889, t1174, t135, t22136, t15740, t18371, t1222, t22175, t1734, t6218, t22169);
    (t72632, t72634, t72648, t72669, t72673, t72703, t72705, t72708, t72727, t72733, t72767, t72798)
}
