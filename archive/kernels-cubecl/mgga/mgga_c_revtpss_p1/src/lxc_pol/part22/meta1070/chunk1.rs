//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3828/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3828<F: Float>(t1424: F, t14299: F, t1444: F, t1904: F, t22386: F, t22415: F, t4071: F, t4076: F, t46353: F, t46356: F, t46359: F, t47764: F, t47772: F, t47777: F, t47781: F, t47784: F, t47786: F, t47791: F, t47909: F, t5728: F, t5775: F, t73587: F, t73590: F, t73593: F, t73598: F) -> F {
    let t73614 = F::cast_from(0.26341796731742046394e1_f64) * t1424 * t4076 * t22386 * t1444 + F::cast_from(0.39274398764404314548e-3_f64) * t47764 - F::cast_from(0.13009920719177044025e-1_f64) * t73587 + F::cast_from(0.10975748638225852664e-1_f64) * t73590 + F::cast_from(0.13009920719177044025e-2_f64) * t73593 + F::cast_from(0.52683593463484092788e1_f64) * t14299 * t5728 + F::cast_from(0.19514881078765566038e-1_f64) * t73598 + F::cast_from(0.22089088168956307394e-3_f64) * t47772 - F::cast_from(0.14634331517634470219e-1_f64) * t46353 - F::cast_from(0.26341796731742046394e1_f64) * t14299 * t5775 - F::cast_from(0.13170898365871023197e1_f64) * t47909 * t1904 + F::cast_from(0.78059524315062264152e-1_f64) * t47777 - F::cast_from(0.39274398764404314548e-3_f64) * t47781 + F::cast_from(0.46263278077393568556e-2_f64) * t47784 - F::cast_from(0.520396828767081761e-2_f64) * t47786 - F::cast_from(0.13009920719177044025e-2_f64) * t46356 + F::cast_from(0.39029762157531132076e-1_f64) * t47791 + t46359 + F::cast_from(0.26341796731742046394e1_f64) * t4071 * t22415;
    t73614
}
