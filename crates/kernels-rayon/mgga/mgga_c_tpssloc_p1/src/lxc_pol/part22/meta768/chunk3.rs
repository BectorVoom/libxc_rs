//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2605/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2605(t1174: f64, t135: f64, t22011: f64, t18375: f64, t5019: f64, t1216: f64, t18946: f64, t19033: f64, t19056: f64, t19083: f64, t22208: f64, t3490: f64, t3506: f64, t44836: f64, t4582: f64, t4950: f64, t4954: f64, t4989: f64, t5030: f64, t65884: f64, t65952: f64, t65992: f64, t65994: f64, t65996: f64, t65998: f64, t72445: f64) -> f64 {
    let t72669 = t1174 * t135 * t22011;
    let t72673 = t5019 * t18375;
    let t72683 = -t44836 * t4582 * t72445 * t1216 / 3072.0_f64 - t65952 / 576.0_f64 + t19083 * t5030 / 144.0_f64 - 5.0_f64 / 5184.0_f64 * t3490 * t22208 + t3506 * t4582 * t19056 * t18946 / 512.0_f64 - 7.0_f64 / 1944.0_f64 * t72669 + 95.0_f64 / 2592.0_f64 * t19033 * t4989 - t72673 / 288.0_f64 - t65992 / 144.0_f64 - t65994 / 144.0_f64 + t65996 / 768.0_f64 + t65998 / 768.0_f64 + t65884 * t4950 / 144.0_f64 + t65884 * t4954 / 144.0_f64;
    t72683
}
