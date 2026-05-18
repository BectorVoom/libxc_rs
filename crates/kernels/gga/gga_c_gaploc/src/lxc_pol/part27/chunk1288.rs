//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1288/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1288<F: Float>(t11013: F, t5771: F, t10972: F, t4614: F, t813: F, t29001: F, t14626: F, t3483: F, t3447: F, t833: F, t2718: F, t8556: F) -> (F, F, F, F, F, F) {
    let t33883 = F::new(0.95334639871601137784e0) * t5771 * t11013;
    let t33891 = F::new(0.12269736305254639897e2) * t813 * t4614 * t10972;
    let t33892 = F::new(0.63904876589867916128e-1) * t29001;
    let t33901 = F::new(0.20449560508757733161e1) * t813 * t14626 * t3483;
    let t33905 = F::new(0.51123901271894332903e1) * t833 * t14626 * t3447;
    let t33907 = F::new(0.47667319935800568892e0) * t2718 * t8556;
    (t33883, t33891, t33892, t33901, t33905, t33907)
}
