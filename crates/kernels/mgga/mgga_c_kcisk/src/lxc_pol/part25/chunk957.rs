//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 957/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk957<F: Float>(t16950: F, t4610: F, t1417: F, t7040: F, t1849: F, t1887: F, t16304: F, t1648: F, t2063: F, t11402: F, t1824: F, t1882: F, t1060: F, t4597: F, t11400: F, t11423: F, t1421: F, t16562: F, t16844: F, t1689: F, t16905: F, t16910: F, t16914: F, t16919: F, t16923: F, t16927: F, t16931: F, t16936: F, t16941: F, t16945: F, t16947: F, t604: F, t6884: F) -> (F,) {
    let t16951 = t16950 * t4610;
    let t16957 = 0.19711289e-2 * t1417 * t7040;
    let t16960 = t1887 * t1849;
    let t16961 = t16960 * t16304;
    let t16964 = t2063 * t1648;
    let t16966 = t11402 * t16964 * t1824;
    let t16969 = t1882 * t1849;
    let t16970 = t16964 * t1060;
    let t16971 = t16969 * t16970;
    let t16974 = t1882 * t4597;
    let t16975 = t16974 * t16970;
    let t16978 = 0.19711289e-2 * t1421 * t16905 + 0.1478346675e-2 * t1421 * t16910 + 0.7391733375e-3 * t1421 * t16914 - 0.295669335e-2 * t1421 * t16919 - 0.1478346675e-2 * t1421 * t16923 - 0.7391733375e-3 * t1421 * t16927 - 0.59133867e-2 * t1421 * t16931 - 0.295669335e-2 * t1421 * t16936 + 0.21901432222222222221e-2 * t16941 - 0.8760572888888888889e-3 * t11423 - t16945 - 0.19711289e-2 * t11400 * t16947 + 0.26281718666666666666e-2 * t11400 * t16951 - 8.0 * t1689 * t6884 + t16957 - 4.0 * t604 * t16562 + 0.26281718666666666666e-2 * t11400 * t16961 - 0.19711289e-2 * t11400 * t16966 - 0.39422578e-2 * t11400 * t16971 + 0.32852148333333333333e-2 * t16844 * t16975;
    (t16978,)
}
