//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 372/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk372<F: Float>(t136: F, t1803: F, t191: F, t1303: F, t203: F, t201: F, t197: F, t118: F, t1386: F, t129: F, t1731: F, t1738: F, t1740: F, t1746: F, t1749: F, t1755: F, t1758: F, t1765: F, t1768: F, t1771: F, t1776: F, t1781: F, t1787: F, t179: F, t1790: F, t1794: F, t1795: F, t1800: F, t182: F, t192: F, t205: F, t578: F, t608: F, t613: F, t629: F, t659: F, t684: F) -> (F, F, F) {
    let t1804 = t1803 * t136;
    let t1805 = t1804 * t191;
    let t1806 = t203 * t1303;
    let t1807 = t201 * t1806;
    let t1808 = t197 * t1807;
    let t1813 = t1386 * t118;
    let t1814 = t1813 * t129;
    let t1819 = -0.27801896084645508334e-2 * t179 * t1731 + 0.28180301985989535023e-7 * t1738 * t1740 - 0.50104576931089393271e-7 * t1746 * t1740 + 0.88531029695126583729e-7 * t1738 * t1749 - 0.15740817079793506587e-6 * t1746 * t1749 - 0.57970906942607043474e-5 * t1755 * t205 - 0.6487109086417285278e-2 * t179 * t1758 - 0.16217772716043213195e-2 * t613 * t1765 + 0.4637672555408563478e-4 * t1768 * t1771 + 0.1081184847736214213e-1 * t179 * t1776 + 0.54106179813099907242e-4 * t629 * t659 - 0.15330084280378307052e-3 * t192 * t1781 - 0.11594181388521408695e-4 * t192 * t1787 + 0.10821235962619981448e-3 * t192 * t1790 + 0.27801896084645508334e-2 * t1794 * t1795 + 0.27801896084645508334e-2 * t179 * t1800 - 0.11594181388521408695e-4 * t1805 * t1808 + 0.2318836277704281739e-4 * t629 * t684 + 0.13900948042322754167e-2 * t1814 * t182 + 0.27801896084645508334e-2 * t578 * t608;
    (t1804, t1808, t1819)
}
