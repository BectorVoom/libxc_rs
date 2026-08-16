//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2605/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2605<F: Float>(t1174: F, t135: F, t22011: F, t18375: F, t5019: F, t1216: F, t18946: F, t19033: F, t19056: F, t19083: F, t22208: F, t3490: F, t3506: F, t44836: F, t4582: F, t4950: F, t4954: F, t4989: F, t5030: F, t65884: F, t65952: F, t65992: F, t65994: F, t65996: F, t65998: F, t72445: F) -> F {
    let t72669 = t1174 * t135 * t22011;
    let t72673 = t5019 * t18375;
    let t72683 = -t44836 * t4582 * t72445 * t1216 / F::cast_from(3072.0_f64) - t65952 / F::cast_from(576.0_f64) + t19083 * t5030 / F::cast_from(144.0_f64) - F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t3490 * t22208 + t3506 * t4582 * t19056 * t18946 / F::cast_from(512.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1944.0_f64) * t72669 + F::cast_from(95.0_f64) / F::cast_from(2592.0_f64) * t19033 * t4989 - t72673 / F::cast_from(288.0_f64) - t65992 / F::cast_from(144.0_f64) - t65994 / F::cast_from(144.0_f64) + t65996 / F::cast_from(768.0_f64) + t65998 / F::cast_from(768.0_f64) + t65884 * t4950 / F::cast_from(144.0_f64) + t65884 * t4954 / F::cast_from(144.0_f64);
    t72683
}
