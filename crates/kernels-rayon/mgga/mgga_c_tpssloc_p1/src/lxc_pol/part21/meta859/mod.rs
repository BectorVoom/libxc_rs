//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta859 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3117;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta859(t1157: f64, t1164: f64, t18785: f64, t3375: f64, t18279: f64, t3378: f64, t1147: f64, t1156: f64, t64425: f64, t15225: f64, t51819: f64, t64482: f64, t18934: f64, t3411: f64, t4882: f64, t51613: f64, t18274: f64, t3404: f64, t300: f64, t63709: f64, t63290: f64, t64475: f64, t64477: f64, t64479: f64, t64481: f64, t64485: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64489, t64492, t64496, t64499) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3117(t1157, t1164, t18785, t3375, t18279, t3378, t1147, t1156, t64425, t15225, t51819, t64482);
        let (t64501, t64504, t64507, t64509, t64510) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3118(t18934, t3411, t1164, t4882, t51613, t18274, t3404, t300, t63709, t63290, t64475, t64477, t64479, t64481, t64485, t64489, t64492, t64496, t64499);
    (t64489, t64492, t64496, t64499, t64501, t64504, t64507, t64509, t64510)
}
