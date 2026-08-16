//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1275;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1276;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta435<F: Float>(t11539: F, t1174: F, t21745: F, t1213: F, t22244: F, t248: F, t3570: F, t1227: F, t21758: F, t45268: F, t11692: F, t11697: F, t22283: F, t11678: F, t22279: F, t22161: F, t3577: F, t19025: F, t5001: F, t22243: F, t486: F, t1222: F, t22116: F, t18332: F, t4889: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t72815, t72849, t72857, t72864) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1275::<F>(t11539, t1174, t21745, t1213, t22244, t248, t3570, t1227, t21758, t45268, t11692, t11697, t22283);
        let (t72936, t72959, t72967, t73028, t73043, t73076) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1276::<F>(t11678, t11697, t22279, t22161, t3577, t19025, t5001, t22243, t486, t1222, t22116, t18332, t4889);
    (t72815, t72849, t72857, t72864, t72936, t72959, t72967, t73028, t73043, t73076)
}
