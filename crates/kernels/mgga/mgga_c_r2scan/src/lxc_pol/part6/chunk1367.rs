//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1367/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1367<F: Float>(t20995: F, t21009: F, t21013: F, t21016: F, t21018: F, t21022: F, t21025: F, t21029: F, t21032: F, t22690: F, t22694: F, t22697: F, t22700: F, t22703: F, t5100: F, t7470: F) -> (F, F) {
    let t25950 = 0.25610080155860322884e1 * t20995 - 0.52396431978519890151e-1 * t21009 - 0.17465477326173296717e-1 * t21013 - 0.97574405393827830186e-2 * t21016 + 0.10401866088065122276e1 * t21018 + 0.34672886960217074253e0 * t21022 + 0.24393601348456957547e-3 * t21025 - 0.26832961483302653301e-2 * t21029 - 0.12805040077930161442e1 * t21032 - 0.14636160809074174528e-1 * t22690 + 0.17465477326173296717e-1 * t22694 + 0.52396431978519890151e-1 * t22697 - 0.17465477326173296717e-1 * t22700 + 0.29272321618148349056e-1 * t22703;
    let t25951 = t5100 * t7470;
    (t25950, t25951)
}
