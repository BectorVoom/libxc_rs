//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 385/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk385(t136: f64, t1803: f64, t191: f64, t1303: f64, t203: f64, t201: f64, t197: f64, t118: f64, t1386: f64, t129: f64, t1731: f64, t1738: f64, t1740: f64, t1746: f64, t1749: f64, t1755: f64, t1758: f64, t1765: f64, t1768: f64, t1771: f64, t1776: f64, t1781: f64, t1787: f64, t179: f64, t1790: f64, t1794: f64, t1795: f64, t1800: f64, t182: f64, t192: f64, t205: f64, t578: f64, t608: f64, t613: f64, t629: f64, t659: f64, t684: f64) -> (f64, f64, f64) {
    let t1804 = t1803 * t136;
    let t1805 = t1804 * t191;
    let t1806 = t203 * t1303;
    let t1807 = t201 * t1806;
    let t1808 = t197 * t1807;
    let t1813 = t1386 * t118;
    let t1814 = t1813 * t129;
    let t1819 = -0.27801896084645508334e-2_f64 * t179 * t1731 + 0.28180301985989535023e-7_f64 * t1738 * t1740 - 0.50104576931089393271e-7_f64 * t1746 * t1740 + 0.88531029695126583729e-7_f64 * t1738 * t1749 - 0.15740817079793506587e-6_f64 * t1746 * t1749 - 0.57970906942607043474e-5_f64 * t1755 * t205 - 0.6487109086417285278e-2_f64 * t179 * t1758 - 0.16217772716043213195e-2_f64 * t613 * t1765 + 0.4637672555408563478e-4_f64 * t1768 * t1771 + 0.1081184847736214213e-1_f64 * t179 * t1776 + 0.54106179813099907242e-4_f64 * t629 * t659 - 0.15330084280378307052e-3_f64 * t192 * t1781 - 0.11594181388521408695e-4_f64 * t192 * t1787 + 0.10821235962619981448e-3_f64 * t192 * t1790 + 0.27801896084645508334e-2_f64 * t1794 * t1795 + 0.27801896084645508334e-2_f64 * t179 * t1800 - 0.11594181388521408695e-4_f64 * t1805 * t1808 + 0.2318836277704281739e-4_f64 * t629 * t684 + 0.13900948042322754167e-2_f64 * t1814 * t182 + 0.27801896084645508334e-2_f64 * t578 * t608;
    (t1804, t1808, t1819)
}
