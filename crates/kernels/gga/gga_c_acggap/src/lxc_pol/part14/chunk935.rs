//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 935/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk935<F: Float>(t35949: F, t35955: F, t35959: F, t35985: F, t36032: F, t36036: F, t36083: F, t36115: F, t36129: F, t36135: F, t36139: F, t36231: F, t36236: F, t36238: F, t36289: F, t36327: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37800 = 0.17149607247227894789e-2 * t35949;
    let t37803 = 0.21437009059034868486e-3 * t35955;
    let t37806 = 0.17149607247227894789e-2 * t35959;
    let t37818 = 0.14291339372689912324e-2 * t35985;
    let t37834 = 0.1324375e0 * t36032;
    let t37835 = 0.1528125e-1 * t36036;
    let t37858 = 0.42874018118069736972e-3 * t36083;
    let t37869 = 0.42874018118069736972e-3 * t36115;
    let t37874 = 0.42874018118069736972e-3 * t36129;
    let t37877 = 0.57165357490759649296e-3 * t36135;
    let t37879 = 0.32012600194825403606e-1 * t36139;
    let t37918 = 0.90702367218671976884e-1 * t36231;
    let t37922 = 0.45351183609335988442e-1 * t36236;
    let t37923 = 0.19055119163586549766e-2 * t36238;
    let t37940 = 0.37737710747524982482e-2 * t36289;
    let t37957 = 0.18868855373762491241e-1 * t36327;
    (t37800, t37803, t37806, t37818, t37834, t37835, t37858, t37869, t37874, t37877, t37879, t37918, t37922, t37923, t37940, t37957)
}
