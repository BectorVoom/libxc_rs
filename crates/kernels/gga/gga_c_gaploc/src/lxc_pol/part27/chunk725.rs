//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 725/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk725<F: Float>(t1339: F, t6519: F, t590: F, t4130: F, t6509: F, t493: F, t2440: F, t524: F, t189: F, t6393: F, t188: F, t1305: F, t2344: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6755 = t1339 * t6519;
    let t6756 = t6755 * t590;
    let t6759 = t4130 * t6509;
    let t6760 = t6759 * t590;
    let t6763 = t493 * t6509;
    let t6764 = t6763 * t590;
    let t6767 = t1339 * t6509;
    let t6768 = t6767 * t590;
    let t6773 = t524 * t2440;
    let t6776 = t189 * t6393;
    let t6777 = t188 * t6776;
    let t6784 = t2344 * t1305;
    (t6756, t6760, t6764, t6767, t6768, t6773, t6776, t6777, t6784)
}
