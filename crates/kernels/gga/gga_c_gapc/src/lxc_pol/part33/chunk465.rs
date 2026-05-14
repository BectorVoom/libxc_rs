//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 465/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk465<F: Float>(t2635: F, t2639: F, t2741: F, t2744: F, t2749: F, t2752: F, t2757: F, t2760: F, t2767: F, t2770: F, t2773: F, t2778: F, t2781: F, t2788: F, t2795: F, t2804: F, t2807: F, t2810: F, t2811: F, t2814: F, t2817: F, t321: F, t326: F, t886: F, t898: F, t899: F, t910: F, t929: F, t934: F, t937: F, t943: F, t954: F, t969: F) -> (F,) {
    let t2820 = 0.19323635647535681158e-6 * t2741 * t943 - 0.343574241813184411e-6 * t2744 * t943 + 0.57970906942607043474e-5 * t934 * t2749 - 0.10821235962619981448e-3 * t326 * t2752 - 0.11594181388521408695e-4 * t326 * t2757 - 0.24657764237740843144e-6 * t2760 * t2767 - 0.69504740211613770836e-4 * t898 * t2770 - 0.69504740211613770836e-4 * t2773 * t899 - 0.38647271295071362317e-6 * t2635 * t2778 - 0.687148483626368822e-6 * t2781 * t2639 + 0.687148483626368822e-6 * t2781 * t2778 + 0.11594181388521408695e-4 * t2788 * t937 - 0.2318836277704281739e-4 * t929 * t954 + 0.2318836277704281739e-4 * t929 * t969 + 0.1081184847736214213e-1 * t321 * t2795 - 0.6487109086417285278e-2 * t886 * t910 - 0.34782544165564226085e-4 * t326 * t2804 + 0.10821235962619981448e-3 * t326 * t2807 + 0.27801896084645508334e-2 * t2810 * t2811 + 0.4637672555408563478e-4 * t2814 * t2817;
    (t2820,)
}
