//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1048/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1048<F: Float>(t35936: F, t35938: F, t35949: F, t35955: F, t35959: F, t35985: F, t36032: F, t36036: F, t36083: F, t36115: F, t36129: F, t36135: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37791 = F::new(0.3973125e0) * t35936;
    let t37792 = F::new(0.264875e0) * t35938;
    let t37800 = F::cast_from(0.17149607247227894789e-2_f64) * t35949;
    let t37803 = F::cast_from(0.21437009059034868486e-3_f64) * t35955;
    let t37806 = F::cast_from(0.17149607247227894789e-2_f64) * t35959;
    let t37818 = F::cast_from(0.14291339372689912324e-2_f64) * t35985;
    let t37834 = F::new(0.1324375e0) * t36032;
    let t37835 = F::new(0.1528125e-1) * t36036;
    let t37858 = F::cast_from(0.42874018118069736972e-3_f64) * t36083;
    let t37869 = F::cast_from(0.42874018118069736972e-3_f64) * t36115;
    let t37874 = F::cast_from(0.42874018118069736972e-3_f64) * t36129;
    let t37877 = F::cast_from(0.57165357490759649296e-3_f64) * t36135;
    (t37791, t37792, t37800, t37803, t37806, t37818, t37834, t37835, t37858, t37869, t37874, t37877)
}
