//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1057/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1057<F: Float>(t23741: F, t3347: F, t2268: F, t26938: F, t6767: F, t21389: F, t7937: F, t10178: F, t6305: F, t6313: F, t20539: F, t2343: F, t2787: F, t3822: F, t21172: F, t2765: F) -> (F, F, F, F, F, F, F) {
    let t31825 = 0.85365019907028448797e-1 * t23741 * t3347;
    let t31835 = 0.68292015925622759036e0 * t2268 * t26938 * t6767;
    let t31838 = 0.68292015925622759036e0 * t2268 * t7937 * t21389;
    let t31840 = 0.34146007962811379518e0 * t6305 * t10178;
    let t31842 = 0.45528010617081839358e0 * t6313 * t10178;
    let t31846 = 0.1138200265427045984e0 * t3822 * t2343 * t2787 * t20539;
    let t31849 = 0.17073003981405689759e0 * t3822 * t2765 * t21172;
    (t31825, t31835, t31838, t31840, t31842, t31846, t31849)
}
