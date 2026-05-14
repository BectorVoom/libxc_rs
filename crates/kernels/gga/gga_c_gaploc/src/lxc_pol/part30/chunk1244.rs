//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1244/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1244<F: Float>(t10455: F, t4950: F, t10140: F, t1572: F, t4673: F, t10348: F, t8155: F, t31770: F, t6824: F, t20367: F, t31775: F, t10537: F, t4379: F, t34407: F, t6716: F, t6717: F) -> (F, F, F, F, F, F, F) {
    let t34900 = 0.95334639871601137784e0 * t4950 * t10455;
    let t34903 = 0.95334639871601137784e0 * t1572 * t4673 * t10140;
    let t34905 = 0.14300195980740170668e1 * t8155 * t10348;
    let t34910 = 0.95334639871601137784e0 * t6824 * t31770;
    let t34912 = 0.47667319935800568892e0 * t20367 * t31775;
    let t34913 = t4379 * t10537;
    let t34914 = 0.59584149919750711116e-1 * t34913;
    let t34917 = 0.13803453343411469884e2 * t6716 * t6717 * t34407;
    (t34900, t34903, t34905, t34910, t34912, t34914, t34917)
}
