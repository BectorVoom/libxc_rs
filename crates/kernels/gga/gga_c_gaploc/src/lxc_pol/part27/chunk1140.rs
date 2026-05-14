//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1140/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1140<F: Float>(t3040: F, t7596: F, t7590: F, t16251: F, t2103: F, t3447: F, t10713: F, t4673: F, t10717: F, t11013: F, t5771: F, t10972: F, t4614: F, t813: F, t29001: F, t14626: F, t3483: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33867 = 0.71500979903700853338e0 * t7596 * t3040;
    let t33869 = 0.35750489951850426669e0 * t7590 * t3040;
    let t33872 = 0.15889106645266856297e0 * t2103 * t16251 * t3447;
    let t33878 = 0.95334639871601137784e0 * t2103 * t4673 * t10713;
    let t33881 = 0.95334639871601137784e0 * t2103 * t4673 * t10717;
    let t33883 = 0.95334639871601137784e0 * t5771 * t11013;
    let t33891 = 0.12269736305254639897e2 * t813 * t4614 * t10972;
    let t33892 = 0.63904876589867916128e-1 * t29001;
    let t33901 = 0.20449560508757733161e1 * t813 * t14626 * t3483;
    (t33867, t33869, t33872, t33878, t33881, t33883, t33891, t33892, t33901)
}
