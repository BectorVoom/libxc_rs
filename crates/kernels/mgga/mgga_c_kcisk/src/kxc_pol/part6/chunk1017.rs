//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1017/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1017<F: Float>(t13091: F, t13092: F, t19543: F, t30592: F, t30595: F, t30599: F, t30603: F, t30613: F, t30617: F, t30623: F, t30626: F, t30629: F, t30632: F, t30635: F) -> F {
    let t30704 = F::new(0.247573125e0) * t30613 - t13091 - t13092 - F::new(0.27595e0) * t19543 + F::new(0.19419375e1) * t30617 + F::new(0.12077e1) * t30595 - F::new(0.181155e1) * t30599 - F::new(0.33547222222222222222e0) * t30592 - F::new(0.301925e0) * t30603 - F::new(0.412621875e-1) * t30623 - F::new(0.36793333333333333333e-1) * t30626 - F::new(0.82785e-1) * t30629 + F::new(0.16557e0) * t30632 - F::new(0.49671e0) * t30635;
    t30704
}
