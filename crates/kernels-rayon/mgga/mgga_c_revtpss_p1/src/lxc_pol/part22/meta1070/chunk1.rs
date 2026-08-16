//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3828/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3828(t1424: f64, t14299: f64, t1444: f64, t1904: f64, t22386: f64, t22415: f64, t4071: f64, t4076: f64, t46353: f64, t46356: f64, t46359: f64, t47764: f64, t47772: f64, t47777: f64, t47781: f64, t47784: f64, t47786: f64, t47791: f64, t47909: f64, t5728: f64, t5775: f64, t73587: f64, t73590: f64, t73593: f64, t73598: f64) -> f64 {
    let t73614 = 0.26341796731742046394e1_f64 * t1424 * t4076 * t22386 * t1444 + 0.39274398764404314548e-3_f64 * t47764 - 0.13009920719177044025e-1_f64 * t73587 + 0.10975748638225852664e-1_f64 * t73590 + 0.13009920719177044025e-2_f64 * t73593 + 0.52683593463484092788e1_f64 * t14299 * t5728 + 0.19514881078765566038e-1_f64 * t73598 + 0.22089088168956307394e-3_f64 * t47772 - 0.14634331517634470219e-1_f64 * t46353 - 0.26341796731742046394e1_f64 * t14299 * t5775 - 0.13170898365871023197e1_f64 * t47909 * t1904 + 0.78059524315062264152e-1_f64 * t47777 - 0.39274398764404314548e-3_f64 * t47781 + 0.46263278077393568556e-2_f64 * t47784 - 0.520396828767081761e-2_f64 * t47786 - 0.13009920719177044025e-2_f64 * t46356 + 0.39029762157531132076e-1_f64 * t47791 + t46359 + 0.26341796731742046394e1_f64 * t4071 * t22415;
    t73614
}
