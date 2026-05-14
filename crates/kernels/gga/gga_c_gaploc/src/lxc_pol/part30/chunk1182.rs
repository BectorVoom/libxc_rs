//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1182/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1182<F: Float>(t10713: F, t2103: F, t4673: F, t10717: F, t11013: F, t5771: F, t10783: F, t10948: F, t1457: F, t32371: F, t33851: F, t33853: F, t33857: F, t33859: F, t33861: F, t33863: F, t33865: F, t33867: F, t33869: F, t33872: F, t7653: F) -> (F,) {
    let t33878 = 0.95334639871601137784e0 * t2103 * t4673 * t10713;
    let t33881 = 0.95334639871601137784e0 * t2103 * t4673 * t10717;
    let t33883 = 0.95334639871601137784e0 * t5771 * t11013;
    let t33887 = t33851 - t33853 - 0.14300195980740170668e1 * t10948 * t7653 - t33857 - t33859 - t33861 + t33863 + t33865 + t33867 + t33869 - t33872 + 0.95334639871601137784e0 * t2103 * t4673 * t10783 + t33878 + t33881 + t33883 + 0.71500979903700853338e0 * t2103 * t1457 * t32371;
    (t33887,)
}
