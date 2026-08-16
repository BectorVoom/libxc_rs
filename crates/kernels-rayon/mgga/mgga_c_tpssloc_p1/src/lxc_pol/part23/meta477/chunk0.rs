//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1429/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1429(t22229: f64, t4869: f64, t6084: f64, t1164: f64, t3400: f64, t3403: f64, t21939: f64, t4874: f64, t1156: f64, t3375: f64, t63332: f64, t63334: f64, t63361: f64, t71142: f64, t71144: f64, t71146: f64, t71152: f64, t77989: f64, t77992: f64, t77995: f64, t78057: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78242 = 0.4155806185363551302e3_f64 * t4869 * t22229;
    let t78243 = t6084 * t6084;
    let t78247 = 0.51947577317044391277e2_f64 * t1164 * t3400 * t78243 * t3403;
    let t78250 = 0.46785788981077169656e1_f64 * t1164 * t4874 * t21939;
    let t78254 = 0.35089341735807877242e1_f64 * t1164 * t3375 * t78243 * t1156;
    let t78266 = -0.31659259259259259258e-1_f64 * t63332 + 0.47488888888888888888e-1_f64 * t63334 + 0.47488888888888888888e-1_f64 * t71142 - 0.14246666666666666667e0_f64 * t71144 + 0.94977777777777777776e-1_f64 * t63361 - 0.42739999999999999999e0_f64 * t78057 - 0.26382716049382716049e-1_f64 * t71146 + 0.4274e0_f64 * t77989 + 0.17808333333333333333e-1_f64 * t77992 - 0.52765432098765432099e-1_f64 * t77995 - 0.14246666666666666667e0_f64 * t71152;
    (t78242, t78243, t78247, t78250, t78254, t78266)
}
