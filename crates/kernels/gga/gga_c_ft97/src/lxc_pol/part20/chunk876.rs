//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 876/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk876<F: Float>(t2506: F, t27836: F, t1434: F, t193: F, t24538: F, t24544: F, t27799: F, t27803: F, t27808: F, t27811: F, t27817: F, t27823: F, t27826: F, t27830: F, t27834: F, t1424: F, t3821: F) -> (F, F, F, F) {
    let t27837 = t2506 * t27836;
    let t27839 = t1434 * t193 * t27837;
    let t27840 = -t27799 / 2.0 + t27803 / 6.0 - t27808 / 3.0 - t24538 + t27811 / 3.0 - t24544 / 18.0 - t27817 / 2.0 - 3.0 / 8.0 * t27823 - 2.0 / 3.0 * t27826 + 2.0 * t27830 + 2.0 * t27834 + t27839;
    let t27841 = t1424 * t3821;
    (t27837, t27839, t27840, t27841)
}
