//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1331/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1331<F: Float>(t2099: F, t2945: F, t9595: F, t9599: F, t2029: F, t9539: F, t17765: F, t1843: F, t2039: F, t2104: F, t2106: F, t2107: F, t25331: F, t25337: F, t26387: F, t2922: F, t2923: F, t2946: F, t300: F, t302: F, t3645: F, t5703: F, t7350: F, t7391: F, t758: F, t7607: F, t761: F, t7664: F, t7700: F, t9258: F, t9589: F, t9591: F, t9594: F, t9596: F, t9600: F) -> (F, F) {
    let t26457 = t2945 * t2099 * t9595;
    let t26460 = t2945 * t2099 * t9599;
    let t26494 = t9539 * t2029;
    let t26499 = 0.54878743191129263326e-1 * t7607 * t9596 - 0.27439371595564631662e-1 * t7607 * t9600 - 0.13719685797782315831e-1 * t7607 * t9591 - 0.68598428988911579158e-2 * t26457 + 0.34299214494455789578e-2 * t26460 - 0.51448821741683684368e-2 * t2945 * t758 * t9594 * t1843 + 0.25724410870841842184e-2 * t2945 * t758 * t2946 * t7350 + 0.25724410870841842184e-2 * t2104 * t300 * t9589 * t2107 + 0.17149607247227894789e-2 * t2922 * t7700 * t2039 * t25331 - 0.85748036236139473944e-3 * t7664 * t25337 * t26387 * t2106 + 0.85748036236139473944e-3 * t2922 * t7700 * t5703 * t3645 - 0.85748036236139473944e-3 * t7664 * t7700 * t17765 * t3645 + 0.25724410870841842183e-2 * t2104 * t9258 * t761 * t7391 - 0.42874018118069736972e-3 * t2922 * t302 * t26494 * t2923;
    (t26494, t26499)
}
