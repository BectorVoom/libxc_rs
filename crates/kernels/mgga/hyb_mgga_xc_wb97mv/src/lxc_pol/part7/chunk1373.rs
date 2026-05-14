//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1373/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1373<F: Float>(t10200: F, t1104: F, t11240: F, t11648: F, t1169: F, t12104: F, t1498: F, t1512: F, t1558: F, t2443: F, t2624: F, t2813: F, t2961: F, t29877: F, t30727: F, t31563: F, t32403: F, t32420: F, t32434: F, t32447: F, t32515: F, t32538: F, t32549: F, t32562: F, t32578: F, t32635: F, t32684: F, t32732: F, t32781: F, t32832: F, t32881: F, t32929: F, t32974: F, t33010: F, t33052: F, t33097: F, t33134: F, t33184: F, t33232: F, t33280: F, t33322: F, t33360: F, t33397: F, t33436: F, t33478: F, t33515: F, t33560: F, t33598: F, t336: F, t33641: F, t33682: F, t33723: F, t33761: F, t33805: F, t33846: F, t33879: F, t33914: F, t33951: F, t3673: F, t3842: F, t4271: F, t432: F, t4492: F, t4519: F, t4642: F, t496: F, t540: F, t919: F, t9610: F, t9707: F) -> (F,) {
    let t33958 = t29877 + t30727 + t31563 * t336 + 2.0 * t11240 * t919 + t4271 * t2443 + t32403 * t432 + 2.0 * t9610 * t1498 + t2624 * t4492 + (t32420 + t32434 + t32447 + t32515 + t32538 + t32549 + t32562 + t32578) * t540 + 2.0 * t11648 * t1169 + t4519 * t2961 + 2.0 * t9707 * t1558 + 4.0 * t3673 * t3842 + 2.0 * t1512 * t10200 + t2813 * t4642 + 2.0 * t1104 * t12104 + t496 * (t33641 + t33805 + t33879 + t33560 + t33682 + t33846 + t33951 + t33761 + t33598 + t33184 + t32974 + t33322 + t33052 + t33397 + t33134 + t32832 + t33478 + t32635 + t33280 + t32929 + t33436 + t33914 + t33515 + t32684 + t33232 + t32781 + t33723 + t32732 + t32881 + t33360 + t33010 + t33097);
    (t33958,)
}
