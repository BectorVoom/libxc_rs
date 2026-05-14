//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1342/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1342<F: Float>(t1592: F, t1632: F, t551: F, t8218: F, t2207: F, t2837: F, t5162: F, t22767: F, t8124: F, t6231: F, t8198: F, t20594: F, t2687: F, t6064: F, t20384: F, t2223: F, t25107: F, t25316: F, t25319: F, t25323: F, t25325: F, t25328: F, t25334: F, t25338: F, t506: F, t529: F) -> (F,) {
    let t25342 = t1592 * t551 * t1632 * t8218;
    let t25345 = t2207 * t2837 * t5162;
    let t25347 = t22767 * t8124;
    let t25350 = t8198 * t6231;
    let t25353 = t20594 * t2687 * t6064;
    let t25355 = -0.34930954652346593433e-1 * t25316 - 0.14636160809074174528e-1 * t25319 + t25323 - 0.17465477326173296717e-1 * t25325 + 0.52396431978519890151e-1 * t25328 + 0.16463622957338778997e0 * t2223 * t529 * t506 * t25107 - 0.20803732176130244552e1 * t25334 - 0.20803732176130244552e1 * t25338 - 0.10401866088065122276e1 * t25342 - 0.52396431978519890151e-1 * t25345 + 0.1590300183910403919e-2 * t25347 + 0.34672886960217074253e0 * t20384 - 0.69345773920434148506e0 * t25350 - 0.1047928639570397803e0 * t25353;
    (t25355,)
}
