//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1048/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1048(t35936: f64, t35938: f64, t35949: f64, t35955: f64, t35959: f64, t35985: f64, t36032: f64, t36036: f64, t36083: f64, t36115: f64, t36129: f64, t36135: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37791 = 0.3973125e0_f64 * t35936;
    let t37792 = 0.264875e0_f64 * t35938;
    let t37800 = 0.17149607247227894789e-2_f64 * t35949;
    let t37803 = 0.21437009059034868486e-3_f64 * t35955;
    let t37806 = 0.17149607247227894789e-2_f64 * t35959;
    let t37818 = 0.14291339372689912324e-2_f64 * t35985;
    let t37834 = 0.1324375e0_f64 * t36032;
    let t37835 = 0.1528125e-1_f64 * t36036;
    let t37858 = 0.42874018118069736972e-3_f64 * t36083;
    let t37869 = 0.42874018118069736972e-3_f64 * t36115;
    let t37874 = 0.42874018118069736972e-3_f64 * t36129;
    let t37877 = 0.57165357490759649296e-3_f64 * t36135;
    (t37791, t37792, t37800, t37803, t37806, t37818, t37834, t37835, t37858, t37869, t37874, t37877)
}
