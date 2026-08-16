//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1275;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1276;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta435(t11539: f64, t1174: f64, t21745: f64, t1213: f64, t22244: f64, t248: f64, t3570: f64, t1227: f64, t21758: f64, t45268: f64, t11692: f64, t11697: f64, t22283: f64, t11678: f64, t22279: f64, t22161: f64, t3577: f64, t19025: f64, t5001: f64, t22243: f64, t486: f64, t1222: f64, t22116: f64, t18332: f64, t4889: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t72815, t72849, t72857, t72864) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1275(t11539, t1174, t21745, t1213, t22244, t248, t3570, t1227, t21758, t45268, t11692, t11697, t22283);
        let (t72936, t72959, t72967, t73028, t73043, t73076) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1276(t11678, t11697, t22279, t22161, t3577, t19025, t5001, t22243, t486, t1222, t22116, t18332, t4889);
    (t72815, t72849, t72857, t72864, t72936, t72959, t72967, t73028, t73043, t73076)
}
