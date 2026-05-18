//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1116/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1116<F: Float>(t21145: F, t21296: F, t21285: F, t1113: F, t1127: F, t1417: F, t1701: F, t17832: F, t17833: F, t17836: F, t17870: F, t17945: F, t17964: F, t17965: F, t17975: F, t21144: F, t21292: F, t21329: F, t2379: F, t2384: F, t30683: F, t4940: F, t4943: F, t4950: F, t4951: F, t4987: F, t5003: F, t5007: F, t5016: F, t5049: F, t52: F, t65735: F, t66098: F, t66154: F, t66451: F, t6757: F, t6784: F, t79305: F, t79430: F, t79457: F, t79892: F, t79951: F, t80012: F, t80149: F, t88314: F, t88384: F, t88393: F) -> (F, F, F) {
    let t88405 = t21296 * t21145;
    let t88413 = t21285 * t21145;
    let t88416 = -F::new(0.11019649358382880326e-4) * t65735 * t88314 + F::new(0.16864243845320605903e-2) * t5007 * t5016 + F::new(0.46477736175058559857e-2) * t17945 * t66451 * t4943 + F::new(0.22023512095983737145e1) * t6784 * t1701 * t80012 * t1113 - F::new(0.11011756047991868572e1) * t1417 * t1701 * t80012 * t1127 + F::new(0.279058811357253504e0) * t17964 * t30683 * t21292 + F::new(0.279058811357253504e0) * t17964 * t6757 * t79457 * t1127 + F::new(0.279058811357253504e0) * t17964 * t6757 * t17965 * t5049 + F::new(0.33081755960978377912e-2) * t79305 * t21292 * t21145 - F::new(0.45048092923603098705e0) * t4987 * t5003 - F::new(0.16540877980489188955e-3) * t79892 * t88384 - F::new(0.52379446938215765024e-2) * t17870 * t4950 * t4951 * t80149 + F::new(0.81118562704294997116e-3) * t4940 * t66154 + F::new(0.20279640676073749279e-3) * t2379 * t88393 * t2384 + F::new(0.10475889387643153005e-1) * t21144 * t4943 * t52 * t17975 - F::new(0.16540877980489188955e-2) * t66098 * t4950 * t4951 * t79430 + F::new(0.55098246791914401631e-4) * t17833 * t88405 - F::new(0.39217632015950386692e-4) * t17836 * t21329 * t4943 * t79951 + F::new(0.11019649358382880326e-3) * t17836 * t17832 * t88413;
    (t88405, t88413, t88416)
}
