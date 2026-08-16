//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 491/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk491(t818: f64, t875: f64, t1086: f64, t2598: f64, t2635: f64, t2639: f64, t2741: f64, t2744: f64, t2749: f64, t2752: f64, t2757: f64, t2760: f64, t2767: f64, t2770: f64, t2773: f64, t2778: f64, t2781: f64, t2788: f64, t2795: f64, t2804: f64, t2807: f64, t2810: f64, t2811: f64, t2814: f64, t321: f64, t326: f64, t886: f64, t898: f64, t899: f64, t910: f64, t929: f64, t934: f64, t937: f64, t943: f64, t954: f64, t969: f64) -> (f64, f64) {
    let t2815 = t818 * t875;
    let t2817 = t1086 * t2598 * t2815;
    let t2820 = 0.19323635647535681158e-6_f64 * t2741 * t943 - 0.343574241813184411e-6_f64 * t2744 * t943 + 0.57970906942607043474e-5_f64 * t934 * t2749 - 0.10821235962619981448e-3_f64 * t326 * t2752 - 0.11594181388521408695e-4_f64 * t326 * t2757 - 0.24657764237740843144e-6_f64 * t2760 * t2767 - 0.69504740211613770836e-4_f64 * t898 * t2770 - 0.69504740211613770836e-4_f64 * t2773 * t899 - 0.38647271295071362317e-6_f64 * t2635 * t2778 - 0.687148483626368822e-6_f64 * t2781 * t2639 + 0.687148483626368822e-6_f64 * t2781 * t2778 + 0.11594181388521408695e-4_f64 * t2788 * t937 - 0.2318836277704281739e-4_f64 * t929 * t954 + 0.2318836277704281739e-4_f64 * t929 * t969 + 0.1081184847736214213e-1_f64 * t321 * t2795 - 0.6487109086417285278e-2_f64 * t886 * t910 - 0.34782544165564226085e-4_f64 * t326 * t2804 + 0.10821235962619981448e-3_f64 * t326 * t2807 + 0.27801896084645508334e-2_f64 * t2810 * t2811 + 0.4637672555408563478e-4_f64 * t2814 * t2817;
    (t2817, t2820)
}
