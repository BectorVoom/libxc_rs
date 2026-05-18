//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1280/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1280<F: Float>(t33094: F, t10847: F, t22706: F, t7584: F, t16455: F, t32889: F, t7585: F, t10948: F, t33067: F, t33069: F, t33072: F, t33074: F, t33077: F, t33079: F, t33080: F, t33081: F, t33084: F, t33090: F, t33092: F, t7736: F) -> F {
    let t33095 = F::new(0.17041300423964777634e0) * t33094;
    let t33098 = F::new(0.30674340763136599742e2) * t7584 * t22706 * t10847;
    let t33101 = F::new(0.23005755572352449806e2) * t16455 * t7585 * t32889;
    let t33102 = -t33067 + t33069 + t33072 + t33074 - t33077 - t33079 - t33080 + t33081 - t33084 - F::new(0.10725146985555128001e1) * t10948 * t7736 + t33090 + t33092 + t33095 - t33098 + t33101;
    t33102
}
