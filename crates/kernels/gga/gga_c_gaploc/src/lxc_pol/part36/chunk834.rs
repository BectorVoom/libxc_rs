//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 834/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk834<F: Float>(t41015: F, t41019: F, t10639: F, t10912: F, t787: F, t899: F, t913: F, t11001: F, t9823: F, t33206: F, t959: F, t13096: F, t1890: F, t1966: F, t40986: F, t40989: F, t43413: F, t43414: F, t43417: F, t43421: F, t43426: F, t43433: F, t43435: F, t43440: F, t43442: F, t43444: F, t43447: F, t43448: F, t590: F) -> (F,) {
    let t43449 = 0.11502877786176224903e1 * t41015;
    let t43450 = 0.46011511144704899612e1 * t41019;
    let t43454 = t787 * t10912 * t899 * t913 * t10639;
    let t43455 = 0.17875244975925213335e0 * t43454;
    let t43456 = t9823 * t11001;
    let t43458 = t33206 * t959;
    let t43460 = -t43413 + t43414 - t43417 + 0.38342925953920749676e0 * t40986 - 0.11502877786176224903e1 * t43421 + 0.72851559312449424384e1 * t40989 - t43426 - 0.51123901271894332902e0 * t1966 * t1890 * t13096 * t590 - t43433 - 0.76685851907841499352e0 * t43435 + t43440 - 0.38342925953920749676e0 * t43442 - t43444 - t43447 + t43448 - t43449 + t43450 - t43455 + 0.71500979903700853338e0 * t43456 + 0.29792074959875355558e-1 * t43458;
    (t43460,)
}
