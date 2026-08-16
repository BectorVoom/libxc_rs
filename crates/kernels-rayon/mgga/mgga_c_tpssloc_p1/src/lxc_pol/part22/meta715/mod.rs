//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta715 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2321;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2322;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2323;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta715(t40741: f64, t40743: f64, t40748: f64, t40760: f64, t40764: f64, t40766: f64, t46292: f64, t67162: f64, t67163: f64, t67169: f64, t67170: f64, t67174: f64, t67176: f64, t67178: f64, t67180: f64, t67183: f64, t67186: f64, t39529: f64, t40779: f64, t40784: f64, t40790: f64, t40793: f64, t40797: f64, t40799: f64, t46311: f64, t67191: f64, t67204: f64, t67206: f64, t67207: f64, t67210: f64, t67211: f64, t67212: f64, t67214: f64, t67215: f64, t46447: f64, t5499: f64, t58972: f64, t12939: f64, t17635: f64, t4195: f64, t20217: f64, t707: f64, t751: f64, t1462: f64, t58976: f64, t39549: f64, t39563: f64, t39585: f64, t39590: f64, t40801: f64, t40803: f64, t67216: f64, t67217: f64, t67226: f64, t67228: f64, t67231: f64, t67244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t67452 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2321(t40741, t40743, t40748, t40760, t40764, t40766, t46292, t67162, t67163, t67169, t67170, t67174, t67176, t67178, t67180, t67183, t67186);
        let t67455 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2322(t39529, t40779, t40784, t40790, t40793, t40797, t40799, t46311, t67191, t67204, t67206, t67207, t67210, t67211, t67212, t67214, t67215);
        let (t67457, t67458, t67461, t67464, t67466, t67467) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2323(t46447, t5499, t58972, t12939, t17635, t4195, t20217, t707, t751, t1462, t58976, t39549, t39563, t39585, t39590, t40801, t40803, t67216, t67217, t67226, t67228, t67231, t67244);
    (t67452, t67455, t67457, t67458, t67461, t67464, t67466, t67467)
}
